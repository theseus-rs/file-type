use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111272661: FileFormat = FileFormat {
    id: 111_272_661,
    source_type: SourceType::Wikidata,
    name: "Ensoniq EPS family compacted disk image",
    extensions: &["eui"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
