use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51994105: FileFormat = FileFormat {
    id: 51_994_105,
    source_type: SourceType::Wikidata,
    name: "IBM DisplayWrite Revisable Form Text File",
    extensions: &["rft"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
