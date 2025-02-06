use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118270989: FileFormat = FileFormat {
    id: 118_270_989,
    source_type: SourceType::Wikidata,
    name: "Altamira Composer file",
    extensions: &["acc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
