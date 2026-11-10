//! Question engine for the «Διαιρετότητα» practice app (Μαθηματικά Α΄ Γυμνασίου).
//!
//! Every question is rebuilt from `(category, seed)`, so the server can check
//! answers without storing anything and without sending answers to the browser.

pub mod gen_basic;
pub mod gen_c_alg;
pub mod gen_c_geo;
pub mod gen_c_misc;
pub mod gen_func;
pub mod gen_geo;
pub mod gen_mkd;
pub mod gen_numbers;
pub mod gen_ratio;
pub mod gen_rational;
pub mod gen_sets;
pub mod lesson;
pub mod mk;
pub mod model;
pub mod theory;
pub mod util;

pub use model::{answer_json, answer_text, check, public, CheckResult, PublicQuestion, Question};
use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Category {
    pub id: &'static str,
    /// Chapter (book unit) this category belongs to.
    pub chapter: &'static str,
    pub title: &'static str,
    pub desc: &'static str,
    /// Book section this practises.
    pub section: &'static str,
    /// Known weak spot from the class test: offered with a highlight and more often.
    pub focus: bool,
}

#[derive(Serialize, Clone, Copy)]
pub struct Chapter {
    pub id: &'static str,
    /// School year: "a" = Α΄ Γυμνασίου, "g" = Γ΄ Γυμνασίου.
    pub grade: &'static str,
    /// Number of the unit in the book.
    pub number: u32,
    pub title: &'static str,
    pub desc: &'static str,
    /// The unit's sections, as the book lists them.
    pub sections: &'static [&'static str],
}

/// Book units, in book order. A new unit needs an entry here and its categories below.
pub const CHAPTERS: &[Chapter] = &[
    Chapter {
        id: "synola",
        grade: "a",
        number: 1,
        title: "Σύνολα",
        desc: "Η έννοια του συνόλου, σχέσεις και πράξεις συνόλων",
        sections: &["Η Έννοια του Συνόλου", "Σχέσεις Συνόλων", "Πράξεις Συνόλων"],
    },
    Chapter {
        id: "arithmoi",
        grade: "a",
        number: 2,
        title: "Αριθμοί",
        desc: "Δυνάμεις, δυαδικό σύστημα, αλγεβρικές παραστάσεις, εξισώσεις",
        sections: &[
            "Δυνάμεις",
            "Συστήματα Αρίθμησης",
            "Αλγεβρικές Παραστάσεις",
            "Αριθμητική Τιμή Αλγεβρικής Παράστασης",
            "Ισότητα – Ιδιότητες Ισοτήτων",
            "Η Έννοια της Εξίσωσης",
        ],
    },
    Chapter {
        id: "diairetotita",
        grade: "a",
        number: 3,
        title: "Διαιρετότητα",
        desc: "Ευκλείδεια διαίρεση, κριτήρια, πρώτοι αριθμοί, ΜΚΔ και ΕΚΠ",
        sections: &[
            "Ευκλείδεια Διαίρεση",
            "Ιδιότητες Διαιρετότητας",
            "Κριτήρια Διαιρετότητας",
            "Πρώτοι και Σύνθετοι Αριθμοί",
            "ΜΚΔ και ΕΚΠ",
            "Δραστηριότητες Ενότητας και Εμπλουτισμού",
        ],
    },
    Chapter {
        id: "rhtoi",
        grade: "a",
        number: 4,
        title: "Ακέραιοι – Ρητοί Αριθμοί",
        desc: "Απόλυτη τιμή, πράξεις με πρόσημα, δυνάμεις, περιοδικοί, εξισώσεις",
        sections: &[
            "Ρητοί Αριθμοί – Απόλυτη Τιμή",
            "Σύγκριση Ρητών Αριθμών",
            "Πρόσθεση και Αφαίρεση Ρητών",
            "Πολλαπλασιασμός και Διαίρεση Ρητών",
            "Δυνάμεις Ρητών Αριθμών",
            "Περιοδικοί Ρητοί Αριθμοί",
            "Επίλυση Εξισώσεων στο Σύνολο των Ρητών",
        ],
    },
    Chapter {
        id: "akolouthies",
        grade: "a",
        number: 5,
        title: "Ακολουθίες",
        desc: "Όροι, κανόνας και γενικός τύπος ακολουθίας",
        sections: &["Ακολουθία", "Γενικός Τύπος Ακολουθίας"],
    },
    Chapter {
        id: "geometria1",
        grade: "a",
        number: 6,
        title: "Βασικές Γεωμετρικές Έννοιες",
        desc: "Μήκη, γωνίες και οι σχέσεις τους, κύκλος",
        sections: &[
            "Μέτρηση Μήκους",
            "Γωνία",
            "Διχοτόμος Γωνίας – Σχέσεις Γωνιών",
            "Κάθετες Ευθείες – Αποστάσεις",
            "Βασικά Στοιχεία Κύκλου",
        ],
    },
    Chapter {
        id: "dianysmata",
        grade: "a",
        number: 7,
        title: "Διανύσματα",
        desc: "Μονόμετρα και διανυσματικά μεγέθη, πράξεις με διανύσματα",
        sections: &["Η Έννοια του Διανύσματος", "Πράξεις με Διανύσματα"],
    },
    Chapter {
        id: "synartiseis",
        grade: "a",
        number: 8,
        title: "Συναρτήσεις",
        desc: "Συντεταγμένες, συνάρτηση και γραφική παράσταση",
        sections: &[
            "Συντεταγμένες Σημείου",
            "Η Έννοια της Αντιστοιχίας – Συνάρτησης",
            "Γραφική Παράσταση Συνάρτησης",
        ],
    },
    Chapter {
        id: "geometria2",
        grade: "a",
        number: 9,
        title: "Γεωμετρία ΙΙ",
        desc: "Παράλληλες ευθείες, γωνίες και στοιχεία τριγώνου",
        sections: &[
            "Παράλληλες Ευθείες που τέμνονται από μια άλλη Ευθεία",
            "Κύρια Στοιχεία Τριγώνου – Σχέσεις Γωνιών Τριγώνου",
            "Δευτερεύοντα Στοιχεία Τριγώνου",
        ],
    },
    Chapter {
        id: "logoi",
        grade: "a",
        number: 10,
        title: "Λόγοι – Αναλογίες",
        desc: "Λόγοι, αναλογίες, κλίμακα και ποσοστά",
        sections: &["Λόγοι – Αναλογίες", "Ιδιότητες Αναλογιών", "Ποσοστά"],
    },
    Chapter {
        id: "statistiki",
        grade: "a",
        number: 11,
        title: "Στατιστική – Πιθανότητες",
        desc: "Μεταβλητές, συχνότητες, διαγράμματα, πιθανότητα",
        sections: &[
            "Μεταβλητές – Είδη Μεταβλητών",
            "Μέθοδοι Παρουσίασης Στατιστικών Δεδομένων",
            "Πείραμα Τύχης – Υπολογισμός Πιθανότητας",
        ],
    },
    Chapter {
        id: "g-taut",
        grade: "g",
        number: 1,
        title: "Αξιοσημείωτες Ταυτότητες",
        desc: "(α ± β)², (α + β)(α − β), (α ± β)³",
        sections: &[
            "Αξιοσημείωτες Ταυτότητες (α ± β)²",
            "Αξιοσημείωτη Ταυτότητα (α + β)(α − β)",
            "Αξιοσημείωτες Ταυτότητες (α ± β)³",
        ],
    },
    Chapter {
        id: "g-parag",
        grade: "g",
        number: 2,
        title: "Παραγοντοποίηση – Ρητές Αλγεβρικές Παραστάσεις",
        desc: "Μέθοδοι παραγοντοποίησης, εξισώσεις 2ου βαθμού, ρητές παραστάσεις",
        sections: &[
            "Μέθοδοι Παραγοντοποίησης",
            "Εξισώσεις Δεύτερου και Ανώτερου Βαθμού",
            "Ρητές Αλγεβρικές Παραστάσεις",
        ],
    },
    Chapter {
        id: "g-geom",
        grade: "g",
        number: 3,
        title: "Γεωμετρία",
        desc: "Ανισοτικές σχέσεις, ισότητα τριγώνων",
        sections: &[
            "Ανισοτικές Σχέσεις στα Τρίγωνα",
            "Ίσα σχήματα – Ισότητα τριγώνων",
            "Κριτήρια Ισότητας Τριγώνων",
            "Κριτήρια Ισότητας Ορθογωνίων Τριγώνων",
        ],
    },
    Chapter {
        id: "g-trig",
        grade: "g",
        number: 4,
        title: "Τριγωνομετρία",
        desc: "Τριγωνομετρικοί αριθμοί, επίλυση τριγώνου",
        sections: &["Τριγωνομετρικοί Αριθμοί", "Επίλυση Τριγώνου"],
    },
    Chapter {
        id: "g-eutheia",
        grade: "g",
        number: 5,
        title: "Ευθεία – Γραμμικά Συστήματα",
        desc: "Απόσταση, μέσο, κλίση, σχετικές θέσεις, συστήματα",
        sections: &[
            "Απόσταση Δύο Σημείων, Μέσο Ευθύγραμμου Τμήματος",
            "Σχετικές Θέσεις Δύο Ευθειών",
            "Γραφική και Αλγεβρική Επίλυση Γραμμικού Συστήματος",
        ],
    },
    Chapter {
        id: "g-stat",
        grade: "g",
        number: 6,
        title: "Στατιστική",
        desc: "Δειγματοληψία, εκτίμηση, μέση τιμή και διάμεσος",
        sections: &["Δειγματοληψία – Εκτίμηση – Γενικεύσεις για τον Πληθυσμό"],
    },
    Chapter {
        id: "g-stereo",
        grade: "g",
        number: 7,
        title: "Στερεομετρία",
        desc: "Πρίσμα, πυραμίδα, κύλινδρος, κώνος, σφαίρα",
        sections: &[
            "Ευθείες και Επίπεδα στον Χώρο",
            "Ορθό Πρίσμα",
            "Κανονική Τετραγωνική Πυραμίδα",
            "Κύλινδρος",
            "Κώνος",
            "Σφαίρα",
        ],
    },
    Chapter {
        id: "g-parab",
        grade: "g",
        number: 8,
        title: "Παραβολή",
        desc: "Η παραβολή y = αx²",
        sections: &["Η Παραβολή y = αx²"],
    },
    Chapter {
        id: "g-tetra",
        grade: "g",
        number: 9,
        title: "Παραλληλόγραμμα – Τραπέζια",
        desc: "Ιδιότητες τετραπλεύρων, ειδικά θεωρήματα στα τρίγωνα",
        sections: &[
            "Παραλληλόγραμμο",
            "Ορθογώνιο Παραλληλόγραμμο",
            "Ρόμβος",
            "Τετράγωνο",
            "Ειδικά Θεωρήματα στα Τρίγωνα",
            "Τραπέζιο",
        ],
    },
];

pub fn chapter_categories(chapter: &str) -> Vec<&'static str> {
    CATEGORIES
        .iter()
        .filter(|c| c.chapter == chapter)
        .map(|c| c.id)
        .collect()
}

pub const CATEGORIES: &[Category] = &[
    Category {
        id: "set-def",
        chapter: "synola",
        title: "Η έννοια του συνόλου",
        desc: "Αναγραφή, ∈ και ∉, πληθικός αριθμός",
        section: "Η Έννοια του Συνόλου",
        focus: false,
    },
    Category {
        id: "set-rel",
        chapter: "synola",
        title: "Σχέσεις συνόλων",
        desc: "Ίσα σύνολα, υποσύνολα",
        section: "Σχέσεις Συνόλων",
        focus: false,
    },
    Category {
        id: "set-ops",
        chapter: "synola",
        title: "Πράξεις συνόλων",
        desc: "Ένωση, τομή, συμπλήρωμα, βέννεια",
        section: "Πράξεις Συνόλων",
        focus: false,
    },
    Category {
        id: "powers",
        chapter: "arithmoi",
        title: "Δυνάμεις",
        desc: "Υπολογισμός και σειρά πράξεων",
        section: "Δυνάμεις",
        focus: false,
    },
    Category {
        id: "base2",
        chapter: "arithmoi",
        title: "Δυαδικό σύστημα",
        desc: "Από και προς το δεκαδικό",
        section: "Συστήματα Αρίθμησης",
        focus: false,
    },
    Category {
        id: "alg",
        chapter: "arithmoi",
        title: "Αλγεβρικές παραστάσεις",
        desc: "Απλοποίηση και μετάφραση φράσεων",
        section: "Αλγεβρικές Παραστάσεις",
        focus: false,
    },
    Category {
        id: "val",
        chapter: "arithmoi",
        title: "Αριθμητική τιμή",
        desc: "Αντικατάσταση μεταβλητών",
        section: "Αριθμητική Τιμή Αλγεβρικής Παράστασης",
        focus: false,
    },
    Category {
        id: "eq",
        chapter: "arithmoi",
        title: "Εξισώσεις",
        desc: "Λύση, επαλήθευση, προβλήματα",
        section: "Η Έννοια της Εξίσωσης",
        focus: false,
    },
    Category {
        id: "eucl",
        chapter: "diairetotita",
        title: "Ευκλείδεια διαίρεση",
        desc: "Δ = δ·π + υ, μέρες και ομάδες",
        section: "Ευκλείδεια Διαίρεση",
        focus: false,
    },
    Category {
        id: "div",
        chapter: "diairetotita",
        title: "Διαιρέτες και πολλαπλάσια",
        desc: "α | β, διαιρέτες ενός αριθμού",
        section: "Ευκλείδεια Διαίρεση",
        focus: false,
    },
    Category {
        id: "prop",
        chapter: "diairetotita",
        title: "Ιδιότητες διαιρετότητας",
        desc: "Άθροισμα, διαφορά, μεταβατική",
        section: "Ιδιότητες Διαιρετότητας",
        focus: false,
    },
    Category {
        id: "crit",
        chapter: "diairetotita",
        title: "Κριτήρια διαιρετότητας",
        desc: "Διαιρείται; Ποιο ψηφίο λείπει;",
        section: "Κριτήρια Διαιρετότητας",
        focus: false,
    },
    Category {
        id: "prime",
        chapter: "diairetotita",
        title: "Πρώτοι και σύνθετοι",
        desc: "Πρώτος ή όχι; Πρώτοι μεταξύ τους;",
        section: "Πρώτοι και Σύνθετοι Αριθμοί",
        focus: false,
    },
    Category {
        id: "fact",
        chapter: "diairetotita",
        title: "Πρώτοι παράγοντες",
        desc: "Ανάλυση με τη «σκάλα»",
        section: "Ανάλυση σε γινόμενο πρώτων παραγόντων",
        focus: false,
    },
    Category {
        id: "pow",
        chapter: "diairetotita",
        title: "ΜΚΔ & ΕΚΠ με δυνάμεις",
        desc: "Μικρότερος ή μεγαλύτερος εκθέτης;",
        section: "ΜΚΔ και ΕΚΠ",
        focus: true,
    },
    Category {
        id: "num",
        chapter: "diairetotita",
        title: "ΜΚΔ & ΕΚΠ αριθμών",
        desc: "Από την ανάλυση στο αποτέλεσμα",
        section: "ΜΚΔ και ΕΚΠ",
        focus: false,
    },
    Category {
        id: "gcdp",
        chapter: "diairetotita",
        title: "Προβλήματα ΜΚΔ",
        desc: "Ομάδες, δέματα, κομμάτια",
        section: "ΜΚΔ και ΕΚΠ",
        focus: false,
    },
    Category {
        id: "time",
        chapter: "diairetotita",
        title: "Προβλήματα ΕΚΠ: ώρες & μέρες",
        desc: "Τρένα, πλοία, δορυφόροι",
        section: "ΜΚΔ και ΕΚΠ",
        focus: true,
    },
    Category {
        id: "years",
        chapter: "diairetotita",
        title: "Προβλήματα ΕΚΠ: χρονιές",
        desc: "Κομήτες και επέτειοι",
        section: "ΜΚΔ και ΕΚΠ",
        focus: false,
    },
    Category {
        id: "chal",
        chapter: "diairetotita",
        title: "Προκλήσεις",
        desc: "Αριθμός ανάμεσα σε δύο όρια",
        section: "Δραστηριότητες Ενότητας",
        focus: false,
    },
    Category {
        id: "enrich",
        chapter: "diairetotita",
        title: "Εμπλουτισμός: κριτήρια 7, 11, 19",
        desc: "Κριτήρια πέρα από το βασικό βιβλίο",
        section: "Δραστηριότητες Εμπλουτισμού",
        focus: false,
    },
    Category {
        id: "rat-abs",
        chapter: "rhtoi",
        title: "Απόλυτη τιμή και σύγκριση",
        desc: "|α|, αντίθετοι, < και >",
        section: "Ρητοί Αριθμοί – Απόλυτη Τιμή · Σύγκριση",
        focus: false,
    },
    Category {
        id: "rat-add",
        chapter: "rhtoi",
        title: "Πρόσθεση και αφαίρεση",
        desc: "Ομόσημοι, ετερόσημοι, − (−α)",
        section: "Πρόσθεση και Αφαίρεση Ρητών",
        focus: false,
    },
    Category {
        id: "rat-mul",
        chapter: "rhtoi",
        title: "Πολλαπλασιασμός και διαίρεση",
        desc: "Κανόνας των προσήμων",
        section: "Πολλαπλασιασμός και Διαίρεση Ρητών",
        focus: false,
    },
    Category {
        id: "rat-pow",
        chapter: "rhtoi",
        title: "Δυνάμεις ρητών",
        desc: "(−2)⁴ ή −2⁴;",
        section: "Δυνάμεις Ρητών Αριθμών",
        focus: false,
    },
    Category {
        id: "rat-ops",
        chapter: "rhtoi",
        title: "Παραστάσεις με ρητούς",
        desc: "Σειρά πράξεων με πρόσημα",
        section: "Πράξεις Ρητών",
        focus: false,
    },
    Category {
        id: "rat-per",
        chapter: "rhtoi",
        title: "Περιοδικοί αριθμοί",
        desc: "Δεκαδικός ή περιοδικός; Σε κλάσμα",
        section: "Περιοδικοί Ρητοί Αριθμοί",
        focus: false,
    },
    Category {
        id: "rat-eq",
        chapter: "rhtoi",
        title: "Εξισώσεις στους ρητούς",
        desc: "Μεταφορά όρων, αρνητικοί συντελεστές",
        section: "Επίλυση Εξισώσεων στο Σύνολο των Ρητών",
        focus: false,
    },
    Category {
        id: "seq-next",
        chapter: "akolouthies",
        title: "Όροι ακολουθίας",
        desc: "Επόμενος όρος, όρος που λείπει",
        section: "Ακολουθία",
        focus: false,
    },
    Category {
        id: "seq-gen",
        chapter: "akolouthies",
        title: "Γενικός τύπος",
        desc: "αν, θέση όρου, μοτίβα",
        section: "Γενικός Τύπος Ακολουθίας",
        focus: false,
    },
    Category {
        id: "geo-len",
        chapter: "geometria1",
        title: "Μήκη και μέσο τμήματος",
        desc: "Μέσο, εξισώσεις, μονάδες",
        section: "Μέτρηση Μήκους",
        focus: false,
    },
    Category {
        id: "geo-angle",
        chapter: "geometria1",
        title: "Γωνίες και σχέσεις γωνιών",
        desc: "Είδη, συμπληρωματικές, παραπληρωματικές, κατακορυφήν",
        section: "Γωνία · Σχέσεις Γωνιών",
        focus: false,
    },
    Category {
        id: "geo-circle",
        chapter: "geometria1",
        title: "Κύκλος",
        desc: "Ακτίνα, διάμετρος, επίκεντρη γωνία",
        section: "Βασικά Στοιχεία Κύκλου",
        focus: false,
    },
    Category {
        id: "vec-concept",
        chapter: "dianysmata",
        title: "Η έννοια του διανύσματος",
        desc: "Μονόμετρα, διανυσματικά, ίσα, αντίθετα",
        section: "Η Έννοια του Διανύσματος",
        focus: false,
    },
    Category {
        id: "vec-ops",
        chapter: "dianysmata",
        title: "Πράξεις με διανύσματα",
        desc: "Άθροισμα, διαφορά, παραλληλόγραμμο",
        section: "Πράξεις με Διανύσματα",
        focus: false,
    },
    Category {
        id: "fn-coord",
        chapter: "synartiseis",
        title: "Συντεταγμένες",
        desc: "Σημεία, τεταρτημόρια",
        section: "Συντεταγμένες Σημείου",
        focus: false,
    },
    Category {
        id: "fn-func",
        chapter: "synartiseis",
        title: "Συνάρτηση",
        desc: "Τιμές, πίνακες, είναι συνάρτηση;",
        section: "Η Έννοια της Αντιστοιχίας – Συνάρτησης",
        focus: false,
    },
    Category {
        id: "fn-graph",
        chapter: "synartiseis",
        title: "Γραφική παράσταση",
        desc: "Σημεία της γραφικής, τομή με τον άξονα y",
        section: "Γραφική Παράσταση Συνάρτησης",
        focus: false,
    },
    Category {
        id: "geo-par",
        chapter: "geometria2",
        title: "Παράλληλες και τέμνουσα",
        desc: "Εντός εναλλάξ, εντός και επί τα αυτά",
        section: "Παράλληλες Ευθείες που τέμνονται από μια άλλη Ευθεία",
        focus: false,
    },
    Category {
        id: "geo-tri",
        chapter: "geometria2",
        title: "Γωνίες τριγώνου",
        desc: "Άθροισμα 180°, εξωτερική, ισοσκελές, είδη",
        section: "Κύρια Στοιχεία Τριγώνου",
        focus: false,
    },
    Category {
        id: "geo-tri2",
        chapter: "geometria2",
        title: "Δευτερεύοντα στοιχεία",
        desc: "Διάμεσος, ύψος, διχοτόμος",
        section: "Δευτερεύοντα Στοιχεία Τριγώνου",
        focus: false,
    },
    Category {
        id: "ratio",
        chapter: "logoi",
        title: "Λόγοι και κλίμακα",
        desc: "Απλοποίηση, μοιρασιά, χάρτες",
        section: "Λόγοι – Αναλογίες",
        focus: false,
    },
    Category {
        id: "analog",
        chapter: "logoi",
        title: "Αναλογίες",
        desc: "Χιαστί γινόμενα, ανάλογα ποσά",
        section: "Ιδιότητες Αναλογιών",
        focus: false,
    },
    Category {
        id: "percent",
        chapter: "logoi",
        title: "Ποσοστά",
        desc: "Ποσοστό, αύξηση, έκπτωση",
        section: "Ποσοστά",
        focus: false,
    },
    Category {
        id: "stat-var",
        chapter: "statistiki",
        title: "Μεταβλητές",
        desc: "Ποιοτική ή ποσοτική;",
        section: "Μεταβλητές – Είδη Μεταβλητών",
        focus: false,
    },
    Category {
        id: "stat-chart",
        chapter: "statistiki",
        title: "Συχνότητες και διαγράμματα",
        desc: "Σχετική συχνότητα, κυκλικό διάγραμμα",
        section: "Μέθοδοι Παρουσίασης Στατιστικών Δεδομένων",
        focus: false,
    },
    Category {
        id: "prob",
        chapter: "statistiki",
        title: "Πιθανότητες",
        desc: "Ζάρια, μπάλες, κάρτες",
        section: "Πείραμα Τύχης – Υπολογισμός Πιθανότητας",
        focus: false,
    },
    Category {
        id: "c-ident",
        chapter: "g-taut",
        title: "Αναπτύγματα και εφαρμογές",
        desc: "Ταυτότητες, υπολογισμοί με ταυτότητες",
        section: "Αξιοσημείωτες Ταυτότητες",
        focus: true,
    },
    Category {
        id: "c-factor",
        chapter: "g-parag",
        title: "Παραγοντοποίηση",
        desc: "Κοινός παράγοντας, ομαδοποίηση, τριώνυμο, κύβοι",
        section: "Μέθοδοι Παραγοντοποίησης",
        focus: true,
    },
    Category {
        id: "c-factor2",
        chapter: "g-parag",
        title: "Σύνθετες παραγοντοποιήσεις",
        desc: "Μέθοδος, αλλαγή προσήμου, ομαδοποίηση + ταυτότητες",
        section: "Μέθοδοι Παραγοντοποίησης",
        focus: true,
    },
    Category {
        id: "c-eq2",
        chapter: "g-parag",
        title: "Εξισώσεις 2ου και ανώτερου βαθμού",
        desc: "Παραγοντοποίηση, αδύνατες, προβλήματα εμβαδού",
        section: "Εξισώσεις Δεύτερου και Ανώτερου Βαθμού",
        focus: false,
    },
    Category {
        id: "c-rational",
        chapter: "g-parag",
        title: "Ρητές αλγεβρικές παραστάσεις",
        desc: "Περιορισμοί, απλοποίηση, ρητές εξισώσεις",
        section: "Ρητές Αλγεβρικές Παραστάσεις",
        focus: true,
    },
    Category {
        id: "c-tri-ineq",
        chapter: "g-geom",
        title: "Ανισοτικές σχέσεις",
        desc: "Τριγωνική ανισότητα, μεγαλύτερη γωνία",
        section: "Ανισοτικές Σχέσεις στα Τρίγωνα",
        focus: false,
    },
    Category {
        id: "c-congr",
        chapter: "g-geom",
        title: "Κριτήρια ισότητας τριγώνων",
        desc: "Π–Π–Π, Π–Γ–Π, Γ–Π–Γ, ορθογώνια",
        section: "Κριτήρια Ισότητας Τριγώνων",
        focus: false,
    },
    Category {
        id: "c-trig-ratio",
        chapter: "g-trig",
        title: "Τριγωνομετρικοί αριθμοί",
        desc: "ημ, συν, εφ, ημ² + συν² = 1",
        section: "Τριγωνομετρικοί Αριθμοί",
        focus: false,
    },
    Category {
        id: "c-trig-solve",
        chapter: "g-trig",
        title: "Επίλυση τριγώνου",
        desc: "Πλευρές και γωνίες με υπολογιστική μηχανή",
        section: "Επίλυση Τριγώνου",
        focus: false,
    },
    Category {
        id: "c-dist",
        chapter: "g-eutheia",
        title: "Απόσταση και μέσο",
        desc: "√, μέσο ευθύγραμμου τμήματος",
        section: "Απόσταση Δύο Σημείων, Μέσο Ευθύγραμμου Τμήματος",
        focus: false,
    },
    Category {
        id: "c-line",
        chapter: "g-eutheia",
        title: "Εξίσωση ευθείας",
        desc: "Κλίση, παράλληλες, κάθετες",
        section: "Σχετικές Θέσεις Δύο Ευθειών",
        focus: false,
    },
    Category {
        id: "c-system",
        chapter: "g-eutheia",
        title: "Γραμμικά συστήματα",
        desc: "Σύστημα 2×2",
        section: "Αλγεβρική Επίλυση Γραμμικού Συστήματος",
        focus: false,
    },
    Category {
        id: "c-sample",
        chapter: "g-stat",
        title: "Δειγματοληψία",
        desc: "Εκτίμηση, αντιπροσωπευτικό δείγμα, μέση τιμή",
        section: "Δειγματοληψία – Εκτίμηση",
        focus: false,
    },
    Category {
        id: "c-solid",
        chapter: "g-stereo",
        title: "Πρίσμα και πυραμίδα",
        desc: "Εμβαδόν και όγκος",
        section: "Ορθό Πρίσμα · Πυραμίδα",
        focus: false,
    },
    Category {
        id: "c-round",
        chapter: "g-stereo",
        title: "Κύλινδρος, κώνος, σφαίρα",
        desc: "Συναρτήσει του π",
        section: "Κύλινδρος · Κώνος · Σφαίρα",
        focus: false,
    },
    Category {
        id: "c-parab",
        chapter: "g-parab",
        title: "Η παραβολή y = αx²",
        desc: "Σημεία, α, μέγιστο/ελάχιστο",
        section: "Η Παραβολή y = αx²",
        focus: false,
    },
    Category {
        id: "c-quad",
        chapter: "g-tetra",
        title: "Παραλληλόγραμμα",
        desc: "Γωνίες, διαγώνιοι, ρόμβος, ιδιότητες",
        section: "Παραλληλόγραμμο · Ρόμβος · Ορθογώνιο",
        focus: false,
    },
    Category {
        id: "c-theorems",
        chapter: "g-tetra",
        title: "Ειδικά θεωρήματα και τραπέζιο",
        desc: "Μέσα πλευρών, διάμεσος, 30°, τραπέζιο",
        section: "Ειδικά Θεωρήματα στα Τρίγωνα · Τραπέζιο",
        focus: false,
    },
];

/// Teacher-facing labels for the mistake tags returned by `check`.
pub const TAGS: &[(&str, &str)] = &[
    (
        "ident-middle",
        "Ταυτότητες: ξέχασε το 2αβ (ή τους μεσαίους όρους)",
    ),
    ("root-sign", "Ρίζες/παράγοντες με λάθος πρόσημο"),
    ("lost-root", "Εξισώσεις: έχασε τη λύση x = 0"),
    ("factor", "Παραγοντοποίηση"),
    ("factor-method", "Αναγνώριση μεθόδου παραγοντοποίησης"),
    (
        "eq-product-const",
        "Εξίσωση γινόμενο = αριθμός: «μηδένισε» τους παράγοντες",
    ),
    ("eq-squares", "Εξίσωση Α² = Β²"),
    ("eq-kind", "Είδος ριζών / αδύνατη εξίσωση"),
    (
        "eq-split",
        "Εξίσωση με κλασματική ρίζα (σπάσιμο μεσαίου όρου)",
    ),
    ("rational-domain", "Ρητές παραστάσεις: περιορισμοί"),
    ("rational-simplify", "Ρητές παραστάσεις: απλοποίηση"),
    ("tri-ineq", "Τριγωνική ανισότητα"),
    ("congr", "Κριτήρια ισότητας τριγώνων"),
    (
        "trig-sides",
        "Τριγωνομετρία: μπέρδεψε απέναντι/προσκείμενη πλευρά",
    ),
    (
        "trig-monotone",
        "Τριγωνομετρία: σύγκριση τριγωνομετρικών αριθμών",
    ),
    ("surd-simplify", "Ρίζα που δεν απλοποιήθηκε"),
    ("dist-sum", "Απόσταση: λάθος στον τύπο"),
    ("midpoint", "Μέσο τμήματος: αφαίρεσε αντί να προσθέσει"),
    ("slope-swap", "Κλίση: αντέστρεψε τον λόγο"),
    ("perp", "Κάθετες ευθείες: ξέχασε το αντίθετο πρόσημο"),
    ("line-pos", "Σχετική θέση ευθειών"),
    ("sys-swap", "Σύστημα: αντέστρεψε x και y"),
    ("sample-bias", "Αντιπροσωπευτικό δείγμα"),
    ("median-sort", "Διάμεσος χωρίς ταξινόμηση"),
    ("cone-third", "Όγκος κώνου/πυραμίδας: ξέχασε το 1/3"),
    ("solid-bases", "Κύλινδρος: ξέχασε μία από τις βάσεις"),
    ("parab-shape", "Παραβολή: μορφή και ακρότατο"),
    ("rhombus-area", "Εμβαδόν ρόμβου: ξέχασε το μισό"),
    ("quad-props", "Ιδιότητες τετραπλεύρων"),
    ("seq-off", "Ακολουθίες: μέτρησε ένα βήμα παραπάνω"),
    ("seq-formula", "Ακολουθίες: γενικός τύπος"),
    ("units", "Μετατροπή μονάδων μέτρησης"),
    ("angle-kind", "Είδη γωνιών"),
    ("comp-supp", "Μπέρδεψε συμπληρωματικές με παραπληρωματικές"),
    ("circle-360", "Κύκλος: ολόκληρος ο κύκλος είναι 360°"),
    ("circle-terms", "Κύκλος: χορδή, διάμετρος, επίκεντρη"),
    ("vec-kind", "Διανύσματα: έννοιες"),
    ("vec-dir", "Διανύσματα: αντίθετη φορά"),
    ("coord-swap", "Συντεταγμένες: αντέστρεψε x και y"),
    ("quadrant", "Τεταρτημόρια"),
    ("fn-def", "Η έννοια της συνάρτησης"),
    (
        "par-angle",
        "Παράλληλες: μπέρδεψε ίσες με παραπληρωματικές γωνίες",
    ),
    ("tri-180", "Τρίγωνο: άθροισμα γωνιών 180°"),
    ("tri-ext", "Τρίγωνο: εξωτερική γωνία"),
    ("tri-kind", "Είδη τριγώνων"),
    ("tri-lines", "Διάμεσος, ύψος, διχοτόμος"),
    ("ratio-order", "Λόγοι: η σειρά των όρων"),
    ("pct-step", "Ποσοστά: σταμάτησε στην αύξηση/έκπτωση"),
    ("pct-base", "Ποσοστά: υπολόγισε το ποσοστό στη λάθος τιμή"),
    ("stat-var", "Είδη μεταβλητών"),
    ("pie-360", "Κυκλικό διάγραμμα: 360° αντί για 100%"),
    ("prob-not", "Πιθανότητα: «να μην είναι»"),
    ("prob-def", "Πιθανότητα: ιδιότητες"),
    (
        "set-elem",
        "Σύνολα: στοιχείο που δεν έχει την ιδιότητα (∈ / ∉)",
    ),
    (
        "set-dup",
        "Σύνολα: μέτρησε δύο φορές στοιχεία που επαναλαμβάνονται",
    ),
    ("set-sub", "Σύνολα: υποσύνολα και ίσα σύνολα"),
    ("set-double", "Σύνολα: δεν αφαίρεσε την τομή στο ν(Α ∪ Β)"),
    ("set-empty", "Σύνολα: έβαλε το ∅ μαζί με στοιχεία"),
    ("set-union-inter", "Σύνολα: μπέρδεψε ένωση με τομή"),
    ("pow-zero", "Δυνάμεις: α⁰ = 1"),
    ("order", "Σειρά των πράξεων"),
    (
        "bin-digit",
        "Δυαδικό: χρησιμοποίησε ψηφία εκτός από 0 και 1",
    ),
    ("bin-rev", "Δυαδικό: έγραψε τα ψηφία ανάποδα"),
    (
        "distrib",
        "Επιμεριστική ιδιότητα: δεν πολλαπλασίασε κάθε όρο",
    ),
    ("alg-words", "Μετάφραση φράσης σε αλγεβρική παράσταση"),
    ("val-sq", "Αριθμητική τιμή: x² αντί για x·x"),
    ("eq-inverse", "Εξισώσεις: λάθος αντίστροφη πράξη"),
    ("abs", "Απόλυτη τιμή αρνητική"),
    ("neg-compare", "Σύγκριση αρνητικών με βάση την απόλυτη τιμή"),
    ("sign", "Λάθος πρόσημο"),
    ("minus-minus", "Αφαίρεση αρνητικού: − (−α) = + α"),
    ("neg-pow", "−2⁴ αντί για (−2)⁴"),
    ("periodic", "Περιοδικοί αριθμοί"),
    ("eq-move", "Εξισώσεις: δεν άλλαξε πρόσημο στη μεταφορά όρου"),
    ("frac-reduce", "Κλάσμα που δεν απλοποιήθηκε"),
    (
        "lcm-min",
        "ΕΚΠ: πήρε τον μικρότερο εκθέτη αντί για τον μεγαλύτερο",
    ),
    (
        "lcm-missing",
        "ΕΚΠ: ξέχασε πρώτο παράγοντα που υπάρχει μόνο σε έναν αριθμό",
    ),
    (
        "gcd-noncommon",
        "ΜΚΔ: έβαλε πρώτο παράγοντα που δεν είναι κοινός",
    ),
    (
        "gcd-max",
        "ΜΚΔ: πήρε τον μεγαλύτερο εκθέτη αντί για τον μικρότερο",
    ),
    (
        "pow-mult",
        "Υπολόγισε δύναμη σαν πολλαπλασιασμό (π.χ. 5² = 10)",
    ),
    ("not-least", "Βρήκε κοινό πολλαπλάσιο αλλά όχι το ελάχιστο"),
    ("not-greatest", "Βρήκε κοινό διαιρέτη αλλά όχι τον μέγιστο"),
    ("gcd-vs-lcm", "Μπέρδεψε ΜΚΔ με ΕΚΠ"),
    (
        "time-remainder",
        "Ώρες σε μέρες: ξέχασε τις ώρες που περισσεύουν",
    ),
    ("time-12", "Μπέρδεψε π.μ. με μ.μ."),
    ("time-day", "Λάθος στη μέτρηση των ημερών"),
    (
        "blank",
        "Πήγε να κάνει έλεγχο χωρίς να απαντήσει σε όλα τα ερωτήματα",
    ),
    ("eucl-rem", "Υπόλοιπο ίσο ή μεγαλύτερο από τον διαιρέτη"),
    (
        "eucl-week",
        "Μέρες της εβδομάδας: ξέχασε τις μέρες που περισσεύουν",
    ),
    ("ceil", "«Πόσα χρειάζονται τουλάχιστον»: ξέχασε το υπόλοιπο"),
    (
        "div-trivial",
        "Διαιρέτες: ξέχασε το 1 ή τον ίδιο τον αριθμό",
    ),
    (
        "div-wrong",
        "Διαιρέτες: διάλεξε αριθμό που δεν είναι διαιρέτης",
    ),
    ("div-missing", "Διαιρέτες: δεν βρήκε όλους τους διαιρέτες"),
    ("div-tf", "Μπέρδεψε «διαιρέτης» με «πολλαπλάσιο»"),
    ("prop", "Ιδιότητες διαιρετότητας"),
    (
        "fact-product",
        "Ανάλυση σε παράγοντες που δεν δίνει τον αρχικό αριθμό",
    ),
    (
        "digits-partial",
        "Συμπλήρωση ψηφίου: δεν βρήκε όλες τις λύσεις",
    ),
    ("crit", "Λάθος σε κριτήριο διαιρετότητας"),
    ("prime-one", "Θεώρησε το 1 πρώτο αριθμό"),
    (
        "prime-comp",
        "Θεώρησε σύνθετο αριθμό πρώτο (ή το αντίστροφο)",
    ),
    ("prime-missed", "Δεν αναγνώρισε κάποιον πρώτο"),
    ("coprime", "Πρώτοι μεταξύ τους"),
    ("enrich", "Κριτήρια εμπλουτισμού (7, 11, 19)"),
    ("range-lcm", "Πρόβλημα με όρια: σταμάτησε στο ΕΚΠ"),
    (
        "range-rem",
        "Πρόβλημα με υπόλοιπο: ξέχασε να προσθέσει το υπόλοιπο",
    ),
    ("years-count", "Μέτρηση φορών: στρογγύλεψε προς τα πάνω"),
];

pub fn generate(cat: &str, seed: u32) -> Option<Question> {
    let mut r = util::Rng::new(
        seed as u64
            ^ ((cat.len() as u64) << 40)
            ^ cat
                .bytes()
                .fold(0u64, |a, b| a.wrapping_mul(131).wrapping_add(b as u64))
                << 8,
    );
    let q = match cat {
        "eucl" => gen_basic::eucl(&mut r),
        "div" => gen_basic::div(&mut r),
        "prop" => gen_basic::prop(&mut r),
        "crit" => gen_basic::crit(&mut r),
        "prime" => gen_basic::prime(&mut r),
        "enrich" => gen_basic::enrich(&mut r),
        "fact" => gen_mkd::fact(&mut r),
        "pow" => gen_mkd::pow(&mut r),
        "num" => gen_mkd::num_q(&mut r),
        "gcdp" => gen_mkd::gcdp(&mut r),
        "time" => gen_mkd::time(&mut r),
        "years" => gen_mkd::years(&mut r),
        "chal" => gen_mkd::chal(&mut r),
        "set-def" => gen_sets::set_def(&mut r),
        "set-rel" => gen_sets::set_rel(&mut r),
        "set-ops" => gen_sets::set_ops(&mut r),
        "powers" => gen_numbers::powers(&mut r),
        "base2" => gen_numbers::base2(&mut r),
        "alg" => gen_numbers::alg(&mut r),
        "val" => gen_numbers::val(&mut r),
        "eq" => gen_numbers::eq(&mut r),
        "rat-abs" => gen_rational::rat_abs(&mut r),
        "rat-add" => gen_rational::rat_add(&mut r),
        "rat-mul" => gen_rational::rat_mul(&mut r),
        "rat-pow" => gen_rational::rat_pow(&mut r),
        "rat-ops" => gen_rational::rat_ops(&mut r),
        "rat-per" => gen_rational::rat_per(&mut r),
        "rat-eq" => gen_rational::rat_eq(&mut r),
        "seq-next" => gen_func::seq_next(&mut r),
        "seq-gen" => gen_func::seq_gen(&mut r),
        "geo-len" => gen_geo::geo_len(&mut r),
        "geo-angle" => gen_geo::geo_angle(&mut r),
        "geo-circle" => gen_geo::geo_circle(&mut r),
        "vec-concept" => gen_func::vec_concept(&mut r),
        "vec-ops" => gen_func::vec_ops(&mut r),
        "fn-coord" => gen_func::fn_coord(&mut r),
        "fn-func" => gen_func::fn_func(&mut r),
        "fn-graph" => gen_func::fn_graph(&mut r),
        "geo-par" => gen_geo::geo_par(&mut r),
        "geo-tri" => gen_geo::geo_tri(&mut r),
        "geo-tri2" => gen_geo::geo_tri2(&mut r),
        "ratio" => gen_ratio::ratio(&mut r),
        "analog" => gen_ratio::analog(&mut r),
        "percent" => gen_ratio::percent(&mut r),
        "stat-var" => gen_ratio::stat_var(&mut r),
        "stat-chart" => gen_ratio::stat_chart(&mut r),
        "prob" => gen_ratio::prob(&mut r),
        "c-ident" => gen_c_alg::c_ident(&mut r),
        "c-factor" => gen_c_alg::c_factor(&mut r),
        "c-eq2" => gen_c_alg::c_eq2(&mut r),
        "c-factor2" => gen_c_alg::c_factor2(&mut r),
        "c-rational" => gen_c_alg::c_rational(&mut r),
        "c-tri-ineq" => gen_c_geo::c_tri_ineq(&mut r),
        "c-congr" => gen_c_geo::c_congr(&mut r),
        "c-trig-ratio" => gen_c_geo::c_trig_ratio(&mut r),
        "c-trig-solve" => gen_c_geo::c_trig_solve(&mut r),
        "c-quad" => gen_c_geo::c_quad(&mut r),
        "c-theorems" => gen_c_geo::c_theorems(&mut r),
        "c-dist" => gen_c_misc::c_dist(&mut r),
        "c-line" => gen_c_misc::c_line(&mut r),
        "c-system" => gen_c_misc::c_system(&mut r),
        "c-sample" => gen_c_misc::c_sample(&mut r),
        "c-solid" => gen_c_misc::c_solid(&mut r),
        "c-round" => gen_c_misc::c_round(&mut r),
        "c-parab" => gen_c_misc::c_parab(&mut r),
        _ => return None,
    };
    Some(q)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn correct_values(q: &Question) -> Vec<Value> {
        q.parts().map(|p| answer_json(&p.answer)).collect()
    }

    #[test]
    fn chapters_and_categories_match() {
        for c in CATEGORIES {
            assert!(
                CHAPTERS.iter().any(|ch| ch.id == c.chapter),
                "{}: unknown chapter",
                c.id
            );
            assert!(generate(c.id, 1).is_some(), "{}: no generator", c.id);
        }
        for ch in CHAPTERS {
            assert!(
                !chapter_categories(ch.id).is_empty(),
                "{}: empty chapter",
                ch.id
            );
        }
    }

    #[test]
    fn every_generator_is_consistent() {
        for c in CATEGORIES {
            for seed in 0..1500u32 {
                let q = generate(c.id, seed).unwrap();
                assert_eq!(q.cat, c.id);
                assert!(q.parts().count() > 0);
                assert!(
                    q.hints.iter().all(|h| !h.is_empty()),
                    "{} {seed}: empty hint",
                    c.id
                );
                // correct answers pass
                let res = check(&q, &correct_values(&q), false);
                assert!(res.correct, "{} seed {seed}: own answer rejected", c.id);
                assert!(res.solution.is_some());
                // deterministic
                let again = generate(c.id, seed).unwrap();
                assert_eq!(q.prompt, again.prompt);
                // wrong answers never panic and never pass
                let junk: Vec<Value> = q.parts().map(|_| Value::from(7)).collect();
                let _ = check(&q, &junk, true);
                let blanks: Vec<Value> = q.parts().map(|_| Value::Null).collect();
                let b = check(&q, &blanks, false);
                assert!(!b.correct && b.solution.is_none());
                // set/select options contain the answer
                for p in q.parts() {
                    match (&p.kind, &p.answer) {
                        (model::Kind::Set { options, .. }, model::Answer::Set(a)) => {
                            assert!(a.iter().all(|x| options.contains(x)))
                        }
                        (model::Kind::Select { options }, model::Answer::Choice(a)) => {
                            assert!(options.iter().any(|(v, _)| v == a))
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    #[test]
    fn known_mistakes_are_diagnosed() {
        // find a pow question and give the classic "min exponent in LCM" answer
        let mut hit = false;
        for seed in 0..200 {
            let q = generate("pow", seed).unwrap();
            let parts: Vec<_> = q.parts().collect();
            let (model::Answer::Pow(g), model::Answer::Pow(_)) =
                (&parts[0].answer, &parts[2].answer)
            else {
                panic!()
            };
            let mut vals = correct_values(&q);
            vals[2] = answer_json(&model::Answer::Pow(g.clone()));
            let res = check(&q, &vals, false);
            if res.diag.iter().any(|d| d.tag == Some("lcm-min")) {
                hit = true;
                break;
            }
        }
        assert!(hit, "lcm-min never diagnosed");

        let mut hit = false;
        for seed in 0..300 {
            let q = generate("time", seed).unwrap();
            if !q.prompt.contains("ώρες") {
                continue;
            }
            let parts: Vec<_> = q.parts().collect();
            let n = parts.len();
            let mut vals = correct_values(&q);
            // keep the start hour: pupil forgot the leftover hours
            let start: i64 = (6..=11)
                .find(|h| q.prompt.contains(&format!("<b>{h}:00 π.μ.")))
                .unwrap();
            let model::Answer::Num(l) = parts[0].answer else {
                panic!()
            };
            if l % 24 == 0 {
                continue;
            }
            let start_day = (0..7)
                .find(|d| q.prompt.contains(&format!("{} στις", util::DAYS[*d])))
                .unwrap() as i64;
            vals[n - 2] = Value::from((start_day + l / 24) % 7);
            vals[n - 1] = Value::from(start);
            let res = check(&q, &vals, false);
            if res.diag.iter().any(|d| d.tag == Some("time-remainder")) {
                hit = true;
                break;
            }
        }
        assert!(hit, "time-remainder never diagnosed");
    }
}
