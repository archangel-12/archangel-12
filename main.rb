# this ruby script was inspired by this repo:  
# "https://github.com/liununu/liununu"  
# please leave him a star, and hopefully, if you see this... umm, well, give me a star too („• ֊ •„)  
# truth be told, i just don't know why i'm using ruby when this could be done in javascript :/  
# but anyway, this is my implementation:

require 'time'

def year_progress
  this_year = Time.now.year

  start_of_year = Time.parse("#{this_year}-01-01T00:00:00Z").to_f * 1000
  end_of_year = Time.parse("#{this_year}-12-31T23:59:59Z").to_f * 1000
  now = Time.now.to_f * 1000

  progress = (now - start_of_year) / (end_of_year - start_of_year)
  progress_percentage = (progress * 100).round(2)

  new_line = "just so you know, #{this_year} is #{progress_percentage}% complete"

  update_readme(new_line)
end

def update_readme(new_line)
  readme_path = "README.md"

  if File.exist?(readme_path)
    content = File.read(readme_path)
    if content.match?(/just so you know, \d{4} is \d+\.\d+% complete/)
      updated_content = content.gsub(/just so you know, \d{4} is \d+\.\d+% complete/, new_line)
    else
      updated_content = content + "\n" + new_line
    end

    File.write(readme_path, updated_content)
    puts "✅ Readme.md updated success :)"
  else
    puts "❌ couldn't find Readme.md :("
  end
end

year_progress