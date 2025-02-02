use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127785591: FileFormat = FileFormat {
    id: 127_785_591,
    source_type: SourceType::Wikidata,
    name: "MetaPost PostScript file",
    extensions: &["mps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
