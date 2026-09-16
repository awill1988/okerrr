const forbidden_attribution =
  /(?:co-authored-by|generated-by|assisted-by|reviewed with claude code|🤖)/i;

module.exports = {
  extends: ['@commitlint/config-conventional'],
  parserPreset: {
    parserOpts: {
      headerPattern:
        /^(\[[A-Z][A-Z0-9]*-[0-9]+\] )?([a-z]+)(?:\(([a-z0-9_/-]+)\))?(!)?: (.+)$/,
      headerCorrespondence: ['ticket', 'type', 'scope', 'breaking', 'subject'],
    },
  },
  rules: {
    'type-enum': [
      2,
      'always',
      ['feat', 'fix', 'refactor', 'chore', 'docs', 'test', 'ci'],
    ],
    'header-max-length': [2, 'always', 72],
    'body-max-line-length': [2, 'always', 72],
    'footer-max-line-length': [2, 'always', 72],
    'subject-full-stop': [2, 'never', '.'],
    'subject-case': [2, 'always', 'lower-case'],
    'header-lowercase': [2, 'always'],
    'no-attribution': [2, 'always'],
  },
  plugins: [
    {
      rules: {
        'header-lowercase': ({ header }) => {
          const without_ticket = header.replace(/^\[[A-Z][A-Z0-9]*-[0-9]+\] /, '');
          return [
            !/[A-Z]/.test(without_ticket),
            'the header must be lowercase after an optional ticket prefix',
          ];
        },
        'no-attribution': ({ raw }) => [
          !forbidden_attribution.test(raw),
          'attribution footers and signatures are not permitted',
        ],
      },
    },
  ],
};
