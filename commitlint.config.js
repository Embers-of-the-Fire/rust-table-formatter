module.exports = {
    extends: ['@commitlint/config-conventional'],
    rules: {
        'body-leading-blank': [2, 'always'], // body换行
        'header-max-length': [2, 'never', 72], // header 最长72
        'type-enum': [
            2,
            'always',
            [
                'add', // 创建功能
                'del', // 删除功能
                'fix', // 解决问题
                'bump', // 修改某个版本号
                'conf', // 配置文件修改
                'refactor', // 必须进行重构的代码
                'reformat', // 代码格式化
                'optimize', // 代码性能优化
                'doc', // 文档构建与修改
                'start', // 开始做某事，比如创建分支等
                'end' // 结束做某事，比如删除分支等
            ]
        ]
    }
}
