import {check} from 'k6';
import http from 'k6/http';

export const options = {
    vus: 2000,
    duration: '20s',
};

export default function () {
    const res = http.get('http://host.docker.internal:8888/questions/10');
    check(res, {
        'is status 200': (r) => r.status === 200,
    });
}