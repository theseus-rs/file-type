use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130542831: FileFormat = FileFormat {
    id: 130_542_831,
    source_type: SourceType::Wikidata,
    name: "Pug file format",
    extensions: &["jade", "pug"],
    media_types: &["text/x-jade", "text/x-pug"],
    signatures: &[],
    related_formats: &[],
};
