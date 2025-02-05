use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_74021741: FileFormat = FileFormat {
    id: 74_021_741,
    source_type: SourceType::Wikidata,
    name: "Microsoft Resource Library",
    extensions: &["rll"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
