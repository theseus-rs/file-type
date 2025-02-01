use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111190518: FileFormat = FileFormat {
    id: 111_190_518,
    puid: "wikidata/111190518",
    name: "Visual Tool Markup Language File",
    extensions: &["vtml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
