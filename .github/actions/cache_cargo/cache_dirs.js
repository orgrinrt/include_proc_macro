const core = require('@actions/core');
const cache = require('@actions/cache');
const glob = require('@actions/glob');

async function run() {
    try {
        const cachePaths = core.getInput('cache-paths', {required: true}).split(';');
        let keyTemplate = core.getInput('key-template', {required: true});
        const cacheInvalidationPattern = core.getInput('cache-invalidation-pattern', {required: true});

        const globber = await glob.create(cacheInvalidationPattern);
        const triggerFiles = await globber.glob();
        const hashObj = require('crypto').createHash('sha1');
        for (const file of triggerFiles) {
            hashObj.update(require('fs').readFileSync(file));
        }
        const hash = '-' + hashObj.digest('hex');

        // Get all placeholders from the template
        const placeholders = keyTemplate.match(/{(.*?)}/g) || [];

        for (let placeholder of placeholders) {
            const inputName = placeholder.slice(1, -1); // Remove braces
            let value = core.getInput(`key-${inputName}`);
            if (!value & inputName === 'prefix') {
                const osType = require('os').platform();
                value = `${osType}-`;
            }
            keyTemplate = keyTemplate.replace(new RegExp(placeholder, 'g'), value);
        };

        for (const path of cachePaths) {
            const key = keyTemplate
                .replace('{path}', path.replace(/[^a-z0-9_]/gi, '_')) // Cleaned path
                .replace('{hash}', hash);

            const cacheKey = await cache.restoreCache([path], key);

            if (!cacheKey) {
                const createdKey = await cache.saveCache([path], key);
                console.log(`Cache created with key: ${createdKey}`);
            } else {
                console.log(`Cache hit on key: ${cacheKey}`);
            }
        }
    } catch (error) {
        core.setFailed(error.message);
    }
}

run();
