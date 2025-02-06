use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117192597: FileFormat = FileFormat {
    id: 117_192_597,
    source_type: SourceType::Wikidata,
    name: "Acrobat TouchUp Image",
    extensions: &["ai", "pdf", "pdp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
