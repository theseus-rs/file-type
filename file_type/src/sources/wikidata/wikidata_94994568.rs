use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_94994568: FileFormat = FileFormat {
    id: 94_994_568,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign ICML",
    extensions: &["icml"],
    media_types: &["application/x-indesign"],
    signatures: &[],
    related_formats: &[],
};
