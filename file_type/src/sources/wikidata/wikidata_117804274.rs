use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117804274: FileFormat = FileFormat {
    id: 117_804_274,
    source_type: SourceType::Wikidata,
    name: "VideoImpression mini-player",
    extensions: &["exe"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
