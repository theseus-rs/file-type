use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76622828: FileFormat = FileFormat {
    id: 76_622_828,
    source_type: SourceType::Wikidata,
    name: "WOLF eBook",
    extensions: &["wol"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
