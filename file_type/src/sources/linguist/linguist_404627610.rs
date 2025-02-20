use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_404627610: FileType = FileType {
    file_format: &FileFormat {
        id: 404_627_610,
        source_type: SourceType::Linguist,
        name: "Gerber Image",
        extensions: &[
            "cmp", "gbl", "gbo", "gbp", "gbr", "gbs", "gko", "gml", "gpb", "gpt", "gtl", "gto",
            "gtp", "gts", "ncl", "sol",
        ],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
