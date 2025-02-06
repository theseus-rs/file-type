use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205685: FileFormat = FileFormat {
    id: 28_205_685,
    source_type: SourceType::Wikidata,
    name: "AMOS Picture Bank",
    extensions: &["abk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
