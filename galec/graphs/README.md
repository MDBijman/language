To watch .dot files and generate .svg files from them on change:

    watchmedo shell-command --patterns="*.dot" --command='dot -Tsvg "${watch_src_path}" -o"${watch_src_path}.svg"'

This command generates .png files, which may not render some characters correctly:

    watchmedo shell-command --patterns="*.dot" --command='dot -Tpng "${watch_src_path}" -o"${watch_src_path}.png"'

Requires `watchmedo` and `graphviz`


