use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111315271: FileFormat = FileFormat {
    id: 111_315_271,
    source_type: SourceType::Wikidata,
    name: "Gravis UltraSnd.ini bank setup",
    extensions: &["ini"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
