use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111272274: FileFormat = FileFormat {
    id: 111_272_274,
    source_type: SourceType::Wikidata,
    name: "Ensoniq KT disk image",
    extensions: &["edk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
