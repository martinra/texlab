use insta::assert_snapshot;
use rowan::ast::AstNode;

use crate::{parser::parse_bibtex, syntax::bibtex};

fn render_entry(input: &str) -> String {
    let green = parse_bibtex(input);
    let root = bibtex::Root::cast(bibtex::SyntaxNode::new_root(green)).unwrap();
    let entry = root.entries().next().unwrap();
    super::render(&entry).unwrap()
}

#[test]
fn article_rivest_1978() {
    assert_snapshot!(render_entry(
        r#"
@article{10.1145/359340.359342,
    author = {Rivest, R. L. and Shamir, A. and Adleman, L.},
    title = {A Method for Obtaining Digital Signatures and Public-Key Cryptosystems},
    year = {1978},
    issue_date = {Feb. 1978},
    publisher = {Association for Computing Machinery},
    address = {New York, NY, USA},
    volume = {21},
    number = {2},
    issn = {0001-0782},
    url = {https://doi.org/10.1145/359340.359342},
    doi = {10.1145/359340.359342},
    journal = {Commun. ACM},
    month = {feb},
    pages = {120-126},
    numpages = {7},
}"#
    ));
}

#[test]
fn article_jain_1999() {
    assert_snapshot!(render_entry(
        r#"
@article{10.1145/331499.331504,
    author = {Jain, A. K. and Murty, M. N. and Flynn, P. J.},
    title = {Data Clustering: A Review},
    year = {1999},
    issue_date = {Sept. 1999},
    publisher = {Association for Computing Machinery},
    address = {New York, NY, USA},
    volume = {31},
    number = {3},
    issn = {0360-0300},
    url = {https://doi.org/10.1145/331499.331504},
    doi = {10.1145/331499.331504},
    journal = {ACM Comput. Surv.},
    month = {sep},
    pages = {264-323},
    numpages = {60},
    keywords = {incremental clustering, clustering applications, exploratory data analysis, cluster analysis, similarity indices, unsupervised learning}
}"#
    ));
}

#[test]
fn article_aksin_2006() {
    assert_snapshot!(render_entry(
        r#"
@string{jomch   = {J.~Organomet. Chem.}}

@article{aksin,
    author       = {Aks{\i}n, {\"O}zge and T{\"u}rkmen, Hayati and Artok, Levent
                    and {\c{C}}etinkaya, Bekir and Ni, Chaoying and
                    B{\"u}y{\"u}kg{\"u}ng{\"o}r, Orhan and {\"O}zkal, Erhan},
    title        = {Effect of immobilization on catalytic characteristics of
                    saturated {Pd-N}-heterocyclic carbenes in {Mizoroki-Heck}
                    reactions},
    journaltitle = jomch,
    date         = 2006,
    volume       = 691,
    number       = 13,
    pages        = {3027-3036},
    indextitle   = {Effect of immobilization on catalytic characteristics},
}"#
    ));
}

#[test]
fn article_betram_1996() {
    assert_snapshot!(render_entry(
        r#"
@string{jams    = {J.~Amer. Math. Soc.}}

@article{bertram,
    author       = {Bertram, Aaron and Wentworth, Richard},
    title        = {Gromov invariants for holomorphic maps on {Riemann} surfaces},
    journaltitle = jams,
    date         = 1996,
    volume       = 9,
    number       = 2,
    pages        = {529-571},
    langid       = {english},
    langidopts   = {variant=american},
    shorttitle   = {Gromov invariants},
    annotation   = {An \texttt{article} entry with a \texttt{volume} and a
                    \texttt{number} field},
}"#
    ));
}

#[test]
fn article_kastenholz_2006() {
    assert_snapshot!(render_entry(
        r#"
@string{jchph   = {J.~Chem. Phys.}}

@article{kastenholz,
    author       = {Kastenholz, M. A. and H{\"u}nenberger, Philippe H.},
    title        = {Computation of methodology\hyphen independent ionic solvation
                    free energies from molecular simulations},
    journaltitle = jchph,
    date         = 2006,
    subtitle     = {{I}. {The} electrostatic potential in molecular liquids},
    volume       = 124,
    eid          = 124106,
    doi          = {10.1063/1.2172593},
    langid       = {english},
    langidopts   = {variant=american},
    indextitle   = {Computation of ionic solvation free energies},
    annotation   = {An \texttt{article} entry with an \texttt{eid} and a
                    \texttt{doi} field. Note that the \textsc{doi} is transformed
                    into a clickable link if \texttt{hyperref} support has been
                    enabled},
}
        "#
    ));
}

#[test]
fn article_blom_2021() {
    assert_snapshot!(render_entry(
        r#"
@article{DBLP:journals/corr/abs-2107-11903,
    author    = {Michelle L. Blom and
                    Jurlind Budurushi and
                    Ronald L. Rivest and
                    Philip B. Stark and
                    Peter J. Stuckey and
                    Vanessa Teague and
                    Damjan Vukcevic},
    title     = {Assertion-based Approaches to Auditing Complex Elections, with application
                    to party-list proportional elections},
    journal   = {CoRR},
    volume    = {abs/2107.11903},
    year      = {2021},
    url       = {https://arxiv.org/abs/2107.11903},
    eprinttype = {arXiv},
    eprint    = {2107.11903},
    timestamp = {Thu, 29 Jul 2021 16:14:15 +0200},
    biburl    = {https://dblp.org/rec/journals/corr/abs-2107-11903.bib},
    bibsource = {dblp computer science bibliography, https://dblp.org}
}"#
    ));
}

#[test]
fn book_aho_2006() {
    assert_snapshot!(render_entry(
        r#"
@book{10.5555/1177220,
    author = {Aho, Alfred V. and Lam, Monica S. and Sethi, Ravi and Ullman, Jeffrey D.},
    title = {Compilers: Principles, Techniques, and Tools (2nd Edition)},
    year = {2006},
    isbn = {0321486811},
    publisher = {Addison-Wesley Longman Publishing Co., Inc.},
    address = {USA}
}"#
    ));
}

#[test]
fn book_averroes_1998() {
    assert_snapshot!(render_entry(
        r#"
@book{averroes/bland,
    author       = {Averroes},
    title        = {The Epistle on the Possibility of Conjunction with the Active
                    Intellect by {Ibn Rushd} with the Commentary of {Moses Narboni}},
    date         = 1982,
    editor       = {Bland, Kalman P.},
    translator   = {Bland, Kalman P.},
    series       = {Moreshet: Studies in {Jewish} History, Literature and Thought},
    number       = 7,
    publisher    = {Jewish Theological Seminary of America},
    location     = {New York},
    keywords     = {primary},
    langid       = {english},
    langidopts   = {variant=american},
    indextitle   = {Epistle on the Possibility of Conjunction, The},
    shorttitle   = {Possibility of Conjunction},
    annotation   = {A \texttt{book} entry with a \texttt{series} and a
                    \texttt{number}. Note the concatenation of the \texttt{editor}
                    and \texttt{translator} fields as well as the
                    \texttt{indextitle} field},
}"#
    ));
}

#[test]
fn book_knuth_1984() {
    assert_snapshot!(render_entry(
        r#"
@book{knuth:ct:a,
    author       = {Knuth, Donald E.},
    title        = {The {\TeX book}},
    date         = 1984,
    maintitle    = {Computers \& Typesetting},
    volume       = {A},
    publisher    = {Addison-Wesley},
    location     = {Reading, Mass.},
    langid       = {english},
    langidopts   = {variant=american},
    sorttitle    = {Computers & Typesetting A},
    indexsorttitle= {The TeXbook},
    indextitle   = {\protect\TeX book, The},
    shorttitle   = {\TeX book},
    annotation   = {The first volume of a five-volume book. Note the
                    \texttt{sorttitle} field. We want this
                    volume to be listed after the entry referring to the entire
                    five-volume set. Also note the \texttt{indextitle} and
                    \texttt{indexsorttitle} fields. Indexing packages that don't
                    generate robust index entries require some control sequences
                    to be protected from expansion},
}
    "#
    ));
}

#[test]
fn mvbook_nietzsche_1988() {
    assert_snapshot!(render_entry(
        r#"
@string{dtv     = {Deutscher Taschenbuch-Verlag}}

@mvbook{nietzsche:ksa,
    author       = {Nietzsche, Friedrich},
    title        = {S{\"a}mtliche Werke},
    date         = 1988,
    editor       = {Colli, Giorgio and Montinari, Mazzino},
    edition      = 2,
    volumes      = 15,
    publisher    = dtv # { and Walter de Gruyter},
    location     = {M{\"u}nchen and Berlin and New York},
    langid       = {german},
    sorttitle    = {Werke-00-000},
    indexsorttitle= {Samtliche Werke},
    subtitle     = {Kritische Studienausgabe},
    annotation   = {The critical edition of Nietzsche's works. This is a
                    \texttt{mvbook} entry referring to a 15-volume work as a
                    whole. Note the \texttt{volumes} field and the format of the
                    \texttt{publisher} and \texttt{location} fields in the
                    database file. Also note the \texttt{sorttitle} and
                    field which is used to fine-tune the
                    sorting order of the bibliography. We want this item listed
                    first in the bibliography},
}"#
    ));
}

#[test]
fn inproceedings_erwin_2007() {
    assert_snapshot!(render_entry(
        r#"
@inproceedings{10.5555/1386993.1386994,
    author = {Erwin, Alva and Gopalan, Raj P. and Achuthan, N. R.},
    title = {A Bottom-up Projection Based Algorithm for Mining High Utility Itemsets},
    year = {2007},
    isbn = {9781920682651},
    publisher = {Australian Computer Society, Inc.},
    address = {AUS},
    booktitle = {Proceedings of the 2nd International Workshop on Integrating Artificial Intelligence and Data Mining - Volume 84},
    pages = {3-11},
    numpages = {9},
    keywords = {pattern growth, high utility itemset mining},
    location = {Gold Coast, Australia},
    series = {AIDM '07}
}"#
    ));
}

#[test]
fn inproceedings_combi_2004() {
    assert_snapshot!(render_entry(
        r#"
@inproceedings{10.1145/967900.968040,
    author = {Combi, Carlo and Pozzi, Giuseppe},
    title = {Architectures for a Temporal Workflow Management System},
    year = {2004},
    isbn = {1581138121},
    publisher = {Association for Computing Machinery},
    address = {New York, NY, USA},
    url = {https://doi.org/10.1145/967900.968040},
    doi = {10.1145/967900.968040},
    booktitle = {Proceedings of the 2004 ACM Symposium on Applied Computing},
    pages = {659-666},
    numpages = {8},
    keywords = {active DBMS, temporal DBMS, workflow management system - WfMS, temporal workflow management system},
    location = {Nicosia, Cyprus},
    series = {SAC '04}
}"#
    ));
}

#[test]
fn collection_matuz_1990() {
    assert_snapshot!(render_entry(
        r#"
@collection{matuz:doody,
    editor       = {Matuz, Roger},
    title        = {Contemporary Literary Criticism},
    year         = 1990,
    volume       = 61,
    publisher    = {Gale},
    location     = {Detroit},
    pages        = {204-208},
    langid       = {english},
    langidopts   = {variant=american},
    annotation   = {A \texttt{collection} entry providing the excerpt information
                    for the \texttt{doody} entry. Note the format of the
                    \texttt{pages} field},
}"#
    ));
}

#[test]
fn patent_almendro_1998() {
    assert_snapshot!(render_entry(
        r#"
@patent{almendro,
    author       = {Almendro, Jos{\'e} L. and Mart{\'i}n, Jacinto and S{\'a}nchez,
                    Alberto and Nozal, Fernando},
    title        = {Elektromagnetisches Signalhorn},
    number       = {EU-29702195U},
    date         = 1998,
    location     = {countryfr and countryuk and countryde},
    langid       = {german},
    annotation   = {This is a \texttt{patent} entry with a \texttt{location}
                    field. The number is given in the \texttt{number} field. Note
                    the format of the \texttt{location} field in the database
                    file. Compare \texttt{laufenberg}, \texttt{sorace}, and
                    \texttt{kowalik}},
}"#
    ));
}
