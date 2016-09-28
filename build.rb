require 'pathname'
require 'fileutils'

Dir["lazers*/**/*md"].each do |source|
  dir, _ = Pathname.new(source).split
  name = Pathname.new(source).basename(".md")

  FileUtils.mkdir_p "doc/#{dir}/#{name}"
  system("pandoc",
         source,
         "--smart",
         "--template",
         "doc/_templates/page.template",
         "-s",
         "-o",
         "doc/#{dir}/#{name}/index.html")
end

system("pandoc",
       "README.md",
       "--smart",
       "--template",
       "doc/_templates/page.template",
       "-s",
       "-o",
       "doc/index.html")