use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111721131: FileFormat = FileFormat {
    id: 111_721_131,
    source_type: SourceType::Wikidata,
    name: "Fixed-format Fortran source",
    extensions: &["f", "f77", "for"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
