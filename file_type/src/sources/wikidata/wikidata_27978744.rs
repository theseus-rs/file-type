use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27978744: FileFormat = FileFormat {
    id: 27_978_744,
    source_type: SourceType::Wikidata,
    name: "DeluxePaint Animation",
    extensions: &["anm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
