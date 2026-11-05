# Regla local para que latexmk genere la lista de símbolos.
add_cus_dep('nlo', 'nls', 0, 'makeindex_nomencl');
sub makeindex_nomencl {
    system("makeindex -s nomencl.ist -o \"$_[0].nls\" \"$_[0].nlo\"") == 0
        or die "makeindex (nomencl) failed";
    return 0;
}
