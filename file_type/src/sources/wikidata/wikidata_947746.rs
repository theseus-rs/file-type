use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_947746: FileFormat = FileFormat {
    id: 947_746,
    source_type: SourceType::Wikidata,
    name: "SREC",
    extensions: &[
        "mot", "s", "s1", "s19", "s2", "s28", "s3", "s37", "srec", "sx",
    ],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
