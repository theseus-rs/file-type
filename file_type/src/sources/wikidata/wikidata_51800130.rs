use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51800130: FileFormat = FileFormat {
    id: 51_800_130,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel Add-In",
    extensions: &["xla", "xll"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
