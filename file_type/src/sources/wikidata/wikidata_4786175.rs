use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4786175: FileFormat = FileFormat {
    id: 4_786_175,
    source_type: SourceType::Wikidata,
    name: "ArchiCAD library part",
    extensions: &["gsm"],
    media_types: &["model/vnd.gs-gdl"],
    signatures: &[],
    related_formats: &[],
};
