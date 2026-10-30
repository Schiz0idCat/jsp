# Reglas locales para que latexmk genere glosario y lista de símbolos.
add_cus_dep('glo', 'gls', 0, 'makeglossaries_custom');
sub makeglossaries_custom {
    system("makeglossaries \"$_[0]\"") == 0 or die "makeglossaries failed";
    return 0;
}

add_cus_dep('nlo', 'nls', 0, 'makeindex_nomencl');
sub makeindex_nomencl {
    system("makeindex -s nomencl.ist -o \"$_[0].nls\" \"$_[0].nlo\"") == 0
        or die "makeindex (nomencl) failed";
    return 0;
}
$out_dir = 'build';
