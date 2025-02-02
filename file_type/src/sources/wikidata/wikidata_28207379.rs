use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207379: FileFormat = FileFormat {
    id: 28_207_379,
    source_type: SourceType::Wikidata,
    name: "TIFF for Fax eXtended",
    extensions: &["tfx", "tif", "tiff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
