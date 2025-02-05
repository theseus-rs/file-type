use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127785591: FileFormat = FileFormat {
    id: 127_785_591,
    source_type: SourceType::Wikidata,
    name: "MetaPost PostScript file",
    extensions: &["mps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
