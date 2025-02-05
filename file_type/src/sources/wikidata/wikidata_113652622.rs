use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113652622: FileFormat = FileFormat {
    id: 113_652_622,
    source_type: SourceType::Wikidata,
    name: "G3 1-D encoded FAX file format",
    extensions: &["fax"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
