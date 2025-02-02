use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111272276: FileFormat = FileFormat {
    id: 111_272_276,
    source_type: SourceType::Wikidata,
    name: "Ensoniq Mirage disk image file",
    extensions: &["edm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
