use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117850207: FileFormat = FileFormat {
    id: 117_850_207,
    source_type: SourceType::Wikidata,
    name: "Xerox MicroFax",
    extensions: &["mif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
