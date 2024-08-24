import test from 'ava'

import { findMatchingFiles, readFileContent } from '../index.js'

test('find_matching_files function pass', (t) => {
  const result = findMatchingFiles(['*.js'], 1);
  t.true(Array.isArray(result));
})

test('read_file_content function pass', (t) => {
  const result = readFileContent('./index.js');
  t.is(typeof result, 'string');
})