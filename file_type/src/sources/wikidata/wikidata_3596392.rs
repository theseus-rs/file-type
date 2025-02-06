use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3596392: FileFormat = FileFormat {
    id: 3_596_392,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word document template",
    extensions: &["dot"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
