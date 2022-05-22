import log from './log.mjs';
import express from 'express';
import * as dotenv from 'dotenv';
dotenv.config();
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();
const app = express();

app.use(express.json({
    type: () => true
}));

async function main() {
    app.post('/submit', async (req, res) => {
        let author = req.body.author;
        let text = req.body.text;
        if (!author || !text) {
            return res.status(400).end();
        }

        await prisma.comment.create({
            data: {
                author,
                text
            }
        });

        res.status(200).end();
    })

    app.get('/', async (req, res) => {
        let allComments = await prisma.comment.findMany();
        let commentCount = allComments.length;
        let template = `There are <strong>${commentCount}</strong> comments.<br>`;
        for (let i of allComments) {
            template += `<strong>${i.author}</strong>: ${i.text}<br>`;
        }

        res.status(200).header('Content-Type', 'text/html').send(template);
    });

    app.listen(process.env.SERVER_PORT, () => {
        log.info(`Now listening on 127.0.0.1:${process.env.SERVER_PORT}`)
    });
}

main().catch(e=>{throw e}).finally(async()=>{await prisma.$disconnect()});