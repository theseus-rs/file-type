use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_94994568: FileFormat = FileFormat {
    id: 94_994_568,
    source_type: SourceType::Wikidata,
    name: "Adobe InDesign ICML",
    extensions: &["icml"],
    media_types: &["application/x-indesign"],
    internal_signatures: &[],
    related_formats: &[],
};
