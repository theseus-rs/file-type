use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111316786: FileFormat = FileFormat {
    id: 111_316_786,
    puid: "wikidata/111316786",
    name: "KAWAI R50/R50E/R50III/R100 ROM-dump",
    extensions: &["kawai12"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
