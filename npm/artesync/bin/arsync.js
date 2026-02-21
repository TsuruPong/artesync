#!/usr/bin/env node

const { spawnSync } = require('child_process');
const os = require('os');
const path = require('path');
const fs = require('fs');

/**
 * Maps Node.js platform strings to our specific package targets
 */
const platformMap = {
	win32: 'win32',
	darwin: 'darwin',
	linux: 'linux',
};

/**
 * Maps Node.js arch strings to our specific package targets
 */
const archMap = {
	x64: 'x64',
	arm64: 'arm64',
};

function getBinaryPackage() {
	const platform = platformMap[os.platform()];
	const arch = archMap[os.arch()];

	if (!platform || !arch) {
		console.error(
			`Unsupported platform or architecture: ${os.platform()}-${os.arch()}`,
		);
		process.exit(1);
	}

	const packageName = `@artesync/${platform}-${arch}`;

	try {
		// We try to locate where the optional dependency was installed
		const packagePath = require.resolve(`${packageName}/package.json`);
		const packageDir = path.dirname(packagePath);

		// Provide the path to the actual executable based on OS extension
		const binaryFile = platform === 'win32' ? 'arsync.exe' : 'arsync';
		const binaryPath = path.join(packageDir, binaryFile);

		if (!fs.existsSync(binaryPath)) {
			console.error(`Binary not found at expected path: ${binaryPath}`);
			process.exit(1);
		}

		return binaryPath;
	} catch (error) {
		console.error(`Failed to locate binary package: ${packageName}`);
		console.error(
			`Please ensure it was correctly installed via optionalDependencies.`,
		);
		process.exit(1);
	}
}

function run() {
	const binaryPath = getBinaryPackage();

	const args = process.argv.slice(2);

	const result = spawnSync(binaryPath, args, { stdio: 'inherit' });

	if (result.error) {
		console.error(result.error);
		process.exit(1);
	}

	process.exit(result.status ?? 0);
}

run();
