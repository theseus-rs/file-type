use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131994277: FileFormat = FileFormat {
    id: 131_994_277,
    source_type: SourceType::Wikidata,
    name: "Web Archive Transformation",
    extensions: &["wat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
