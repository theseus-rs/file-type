use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4027909: FileFormat = FileFormat {
    id: 4_027_909,
    source_type: SourceType::Wikidata,
    name: "Network Bootable Image",
    extensions: &["nbi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
