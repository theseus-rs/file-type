use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1593782: FileFormat = FileFormat {
    id: 1_593_782,
    puid: "wikidata/1593782",
    name: "FASTA format",
    extensions: &["fa", "fa", "fa", "fasta", "fasta", "fasta"],
    media_types: &[
        "chemical/seq-aa-fasta",
        "chemical/seq-aa-fasta",
        "chemical/seq-na-fasta",
        "chemical/seq-na-fasta",
        "text/plain",
        "text/plain",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
