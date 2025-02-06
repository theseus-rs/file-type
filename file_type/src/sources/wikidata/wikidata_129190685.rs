use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129190685: FileFormat = FileFormat {
    id: 129_190_685,
    source_type: SourceType::Wikidata,
    name: "FStar file format",
    extensions: &["fst"],
    media_types: &["text/x-fstar"],
    signatures: &[],
    related_formats: &[],
};
