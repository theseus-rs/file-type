use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1593782: FileFormat = FileFormat {
    id: 1_593_782,
    source_type: SourceType::Wikidata,
    name: "FASTA format",
    extensions: &["fa", "fasta"],
    media_types: &[
        "chemical/seq-aa-fasta",
        "chemical/seq-na-fasta",
        "text/plain",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
