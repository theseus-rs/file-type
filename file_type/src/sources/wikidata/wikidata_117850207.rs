use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117850207: FileFormat = FileFormat {
    id: 117_850_207,
    source_type: SourceType::Wikidata,
    name: "Xerox MicroFax",
    extensions: &["mif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
