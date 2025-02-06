use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60371302: FileFormat = FileFormat {
    id: 60_371_302,
    source_type: SourceType::Wikidata,
    name: "Microsoft PowerPoint Macro-Enabled Show",
    extensions: &["ppsm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
