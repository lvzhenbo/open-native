import test from 'ava'
import { fileURLToPath } from 'node:url'
import { dirname } from 'node:path'

import { open, openWith, reveal } from '../index'

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

// 检测是否在无桌面环境（如 Docker/CI）中运行
const isHeadless = process.env.CI || !process.env.DISPLAY

test('open throws error for non-existent path', (t) => {
  const error = t.throws(() => open('/non/existent/path/file.txt'))
  t.true(error?.message.includes('Path does not exist'))
})

test('reveal throws error for non-existent path', (t) => {
  const error = t.throws(() => reveal('/non/existent/path/file.txt'))
  t.true(error?.message.includes('Path does not exist'))
})

test('openWith throws error for non-existent path', (t) => {
  const error = t.throws(() => openWith('/non/existent/path/file.txt', 'notepad'))
  t.true(error?.message.includes('Path does not exist'))
})

test('open existing directory should work', (t) => {
  if (isHeadless && process.platform === 'linux') {
    t.pass('Skipped in headless Linux environment')
    return
  }
  // 打开当前测试目录，这应该能正常工作
  t.notThrows(() => open(__dirname))
})

test('reveal existing file should work', (t) => {
  if (isHeadless && process.platform === 'linux') {
    t.pass('Skipped in headless Linux environment')
    return
  }
  // reveal 当前测试文件
  t.notThrows(() => reveal(__filename))
})
