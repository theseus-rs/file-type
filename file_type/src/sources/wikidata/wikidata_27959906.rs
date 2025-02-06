use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959906: FileFormat = FileFormat {
    id: 27_959_906,
    source_type: SourceType::Wikidata,
    name: "Super Studio Session song",
    extensions: &["sss"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
