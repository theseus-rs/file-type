use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127265031: FileFormat = FileFormat {
    id: 127_265_031,
    source_type: SourceType::Wikidata,
    name: "ANSYS input file",
    extensions: &["cdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
