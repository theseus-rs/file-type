use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66662117: FileFormat = FileFormat {
    id: 66_662_117,
    source_type: SourceType::Wikidata,
    name: "Lotus Ami Pro Styles",
    extensions: &["sty"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
