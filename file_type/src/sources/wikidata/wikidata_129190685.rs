use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129190685: FileFormat = FileFormat {
    id: 129_190_685,
    source_type: SourceType::Wikidata,
    name: "FStar file format",
    extensions: &["fst"],
    media_types: &["text/x-fstar"],
    internal_signatures: &[],
    related_formats: &[],
};
