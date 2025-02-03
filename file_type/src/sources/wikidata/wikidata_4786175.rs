use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4786175: FileFormat = FileFormat {
    id: 4_786_175,
    source_type: SourceType::Wikidata,
    name: "ArchiCAD library part",
    extensions: &["gsm"],
    media_types: &["model/vnd.gs-gdl"],
    internal_signatures: &[],
    related_formats: &[],
};
