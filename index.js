const fs = require('fs');

const formatter = (line) => {
    const pieces = line.split(/\"GET\s/);
    const uriEtc = pieces[1].split(/\:80/)[1];
    const uri = uriEtc.split(/HTTP/)[0];
    return uri;
};

fs.readdir('./logs', (err, files) => {
    files.forEach((fileName) => {
        fs.readFile(`./logs/${ fileName }`, (err, data) => {
            const lines = data.toString().split('\n');
            lines.forEach((line) => {
                if (line.match(/GET/) && line.match(/t_product/)) {
                    fs.appendFileSync('data.csv', `${ formatter(line) }\n`);
                }
            });
        });
    })
});
