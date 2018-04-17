const fs = require('fs'),
    R = require('ramda'),
    async = require('async');

const formatter = (line) => {
    const pieces = line.split(/\"GET\s/);
    const uriEtc = pieces[1].split(/\:80/)[1];
    const uri = uriEtc.split(/HTTP/)[0];
    return uri;
};

const readFiles = (cb) => {
    fs.readdir('./logs', cb);
};

const loadLines = (files, cb) => {
    const lines = [];
    const reader = (fileName, cb) => {
        fs.readFile(`./logs/${ fileName }`, (err, data) => {
            const rawlines = data.toString().split('\n');

            rawlines.forEach((line) => {
                if (line.match(/GET/)) {
                    lines.push(`${ formatter(line) }\n`);
                }
            });

            return cb(err);
        });
    };

    async.each(files, reader, (err) => {
        if (err) {
            return cb(err);
        }
        return cb(null, lines);
    });   
};

const dedupLines = (lines, cb) => {
    console.log('Before dedup: ', lines.length);

    const dedup = R.uniq(lines);

    console.log('After: ', dedup.length);

    return cb(null, dedup);
};

const writeLines = (lines, cb) => {
    const writer = (line, cb) => {
        fs.appendFile('data.csv', line, cb);
    };

    async.each(lines, writer, cb)
};

async.waterfall([readFiles, loadLines, dedupLines, writeLines])