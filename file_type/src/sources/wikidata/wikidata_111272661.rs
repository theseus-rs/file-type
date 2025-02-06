use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111272661: FileFormat = FileFormat {
    id: 111_272_661,
    source_type: SourceType::Wikidata,
    name: "Ensoniq EPS family compacted disk image",
    extensions: &["eui"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
